import { copyFile, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const SUPPORTED_VERSIONS = [
    {
        pvn: 769,
        version: "1.21.4",
    },
    {
        pvn: 768,
        version: "1.21.2",
    },
    {
        pvn: 767,
        version: "1.21",
    },
    {
        pvn: 766,
        version: "1.20.5",
    },
    {
        pvn: 765,
        version: "1.20.3",
    },
    {
        pvn: 764,
        version: "1.20.2",
    },
    {
        pvn: 763,
        version: "1.20",
    },
];

function generateLauncherProfiles() {
    const minecraftHome = join(process.env.HOME, ".other_minecraft");
    return SUPPORTED_VERSIONS.map(({ pvn, version }) => {
        const created = new Date().toISOString();
        return [
            crypto.randomUUID().replaceAll("-", ""),
            {
                created,
                icon: "Furnace",
                lastUsed: created,
                lastVersionId: version,
                name: `${version} (${pvn})`,
                gameDir: minecraftHome,
                resolution: {
                    height: 480,
                    width: 854,
                },
                type: "custom",
            },
        ];
    });
}

(async () => {
    const minecraftHome = join(process.env.HOME, ".minecraft");
    const launcherProfiles = join(minecraftHome, "launcher_profiles.json");
    await copyFile(launcherProfiles, `${launcherProfiles}.backup`);
    const currentProfiles = JSON.parse(
        await readFile(launcherProfiles, "utf8"),
    );
    const profilesToAdd = Object.fromEntries(generateLauncherProfiles());
    currentProfiles["profiles"] = {
        ...currentProfiles["profiles"],
        ...profilesToAdd,
    };
    await writeFile(launcherProfiles, JSON.stringify(currentProfiles, null, 2));
})();
