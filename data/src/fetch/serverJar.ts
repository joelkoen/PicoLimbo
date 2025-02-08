import {join} from "node:path";
import {mkdir, stat, writeFile} from "node:fs/promises";

type VersionManifest = {
    versions: {
        id: string;
        url: string;
    }[];
};

type Version = {
    downloads: {
        server: {
            url: string;
            sha1: string;
            size: number;
        };
    };
};

const HOSTS = {
    versionManifests:
        "https://launchermeta.mojang.com/mc/game/version_manifest.json",
};

const api = <T>(url: string): Promise<T> =>
    new Promise((resolve) => {
        fetch(url)
            .then((res) => res.json())
            .then((data) => resolve(data));
    });

async function downloadJar(
    downloadUrl: string,
    savePath: string,
): Promise<void> {
    const serverDownload = (await api<Version>(downloadUrl)).downloads.server;
    const blob = await fetch(serverDownload.url).then((res) => res.blob());
    await writeFile(savePath, await blob.bytes());
}

type ServerJar = {
    version: string;
    path: string;
    fileName: string;
    exists: boolean;
};

export async function downloadServerJars(
    versionsToDownload: string[],
    savePath: string,
): Promise<ServerJar[]> {
    if (!(await fileExists(savePath))) {
        await mkdir(savePath, {recursive: true});
    }

    // Check if all Jars are already downloaded
    const promises = versionsToDownload.map(async (version) => {
        const fileName = `${version}.jar`;
        const path = join(savePath, fileName);
        const exists = await fileExists(path);
        return {fileName, path, exists, version};
    });

    const result = await Promise.all(promises);
    if (result.every((e) => e.exists)) {
        return result;
    }

    // If at least one does not exist, start the download
    const versions = (await api<VersionManifest>(HOSTS.versionManifests))
        .versions.filter(it => versionsToDownload.includes(it.id));

    const serverJars = versions.map(async (version) => {
        const fileName = `${version.id}.jar`;
        const serverJarPath = join(savePath, fileName);
        if (!(await fileExists(serverJarPath))) {
            await downloadJar(version.url, serverJarPath);
        }
        return {path: serverJarPath, version: version.id, fileName, exists: true};
    });

    return Promise.all(serverJars);
}

const fileExists = async (path: string) =>
    !!(await stat(path).catch((_) => false));
