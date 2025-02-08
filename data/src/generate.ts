import {writeFile, mkdtemp, copyFile, rm, mkdir, readdir} from "node:fs/promises";
import {join, dirname} from "node:path";
import {exec} from "node:child_process";

const api = <T>(url: string): Promise<T> =>
    new Promise((resolve) => {
        fetch(url)
            .then((res) => res.json())
            .then((data) => resolve(data));
    });

const execute = async (command: string, cwd: string): Promise<string> =>
    new Promise((resolve, reject) => {
        exec(command, {cwd}, (error, stdout, stderr) => {
            if (error) {
                return reject(error);
            }
            if (stderr) {
                return reject(stderr);
            }
            return resolve(stdout);
        });
    });

const HOSTS = {
    versionManifests:
        "https://launchermeta.mojang.com/mc/game/version_manifest.json",
};

const SUPPORTED_VERSIONS = ["1.21.4", "1.21.2", "1.21"];

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

(async () => {
    const versions = (
        await api<VersionManifest>(HOSTS.versionManifests)
    ).versions.filter((version) => SUPPORTED_VERSIONS.includes(version.id));

    const serverJarDirectory = await mkdtemp("/tmp/servers");

    for (const version of versions) {
        // Download server jar file
        const serverDownload = (await api<Version>(version.url)).downloads.server;
        const blob = await fetch(serverDownload.url).then((res) => res.blob());
        const jarFileName = `${version.id}.jar`;
        const serverJarPath = join(serverJarDirectory, jarFileName);
        await writeFile(serverJarPath, await blob.bytes());

        // Run the server to output the files
        const generatedDirectory = await mkdtemp(`/tmp/generated_${version.id}`);
        await execute(
            `java -DbundlerMainClass=net.minecraft.data.Main -jar ${jarFileName} --reports --server --output ${generatedDirectory}`,
            serverJarDirectory,
        );
        console.log(`Generated ${version.id}: ${serverJarPath}`);

        // Copy the generated data
        const outputDirectory = join(process.cwd(), "generated", `V${version.id.replaceAll(".", "_")}`);
        await move(generatedDirectory, outputDirectory, "data");
        await move(generatedDirectory, outputDirectory, "reports");
        await rm(generatedDirectory, {recursive: true, force: true});
    }

    await rm(serverJarDirectory, {recursive: true, force: true});
})();

const move = async (
    from: string,
    to: string,
    subdir: string,
): Promise<void> => {
    await copyDir(join(from, subdir), join(to, subdir));
};

async function copyDir(src: string, dest: string): Promise<void> {
    const entries = await readdir(src, {recursive: true, withFileTypes: true});

    for (const entry of entries) {
        const srcPath = join(entry.parentPath, entry.name);
        const destPath = srcPath.replace(src, dest);
        const destDir = dirname(destPath);

        if (entry.isFile()) {
            await mkdir(destDir, {recursive: true});
            await copyFile(srcPath, destPath);
        }
    }
}
