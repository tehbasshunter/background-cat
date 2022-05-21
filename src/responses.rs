use indoc::indoc;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref RESPONSES: HashMap<&'static str, &'static str> = HashMap::from([
        (
            "program-files",
            indoc! {"
                Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\n\
                You should move it somewhere else, like your Desktop."
            }
        ),
        (
            "macos-java-too-new",
            indoc! {
            "You are using too new a Java version. Please follow the steps on this wiki page to install 8u241: https://github.com/MultiMC/MultiMC5/wiki/Java-on-macOS"
            }
        ),
        (
            "id-limit",
            indoc! {
                    "You've exceeded the hardcoded ID Limit. Remove some mods, or install [JustEnoughIDs](https://www.curseforge.com/minecraft/mc-mods/jeid)"
            }
        ),
        (
            "out-of-memory",
            indoc! {
                "You've run out of memory. You should allocate more, although the exact value depends on how many mods you have installed. \
                [Click this link for a guide.](https://cdn.discordapp.com/attachments/531598137790562305/575376840173027330/unknown.png)"
            }
        ),
        (
            "optifine-and-shadermod",
            indoc! {
                "You've installed Shaders Mod alongside OptiFine. OptiFine has built-in shader support, so you should remove Shaders Mod"
            }
        ),
        (
            "missing-fabric-api",
            indoc! {
                "You are missing Fabric API, which is required by a mod. \
                [Download the needed version here](https://www.curseforge.com/minecraft/mc-mods/fabric-api)"
            }
        ),
        (
            "multimc-in-onedrive",
            indoc! {
                "MultiMC is located in a folder managed by OneDrive. OneDrive messes with Minecraft folders while the game is running, \
                and this often leads to crashes.
                You should move the MultiMC folder to a different folder."
            }
        ),
        (
            "use-java-8",
            indoc! {
                "The version of Minecraft you are playing does not support using modern versions of Java. \
                [Please use Java 8, click here for help.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)"
            }
        ),
        (
            "use-java-17",
            indoc! {
                "You are playing a version of Minecraft that requires Java 17, but are using an older Java version.
                Please install Java 17 you can find downloads [here](https://www.azul.com/downloads/?version=java-17-lts&architecture=x86-64-bit&package=jre)
                On Windows: Download the .msi file. After installation you may have to update MultiMC to detect the new Java version, to do so open the settings and \
                change Update Channel to 'Development', then update MultiMC.
                Open the MultiMC Java settings and make sure Java 8 is still selected as default for more help with that run `-sjava`.\
                Then edit your 1.17+ instance settings, open the Java tab, check 'Java Installation', click 'Auto-detect..' and select Java 17."
            }
        ),
        (
            "apple-silicon-incompatible-forge",
            indoc! {
                "You seem to be using an Apple M1 Mac with an incompatible version of Forge. Add the following to your launch arguments as a workaround: `-Dfml.earlyprogresswindow=false`"
            }
        ),
        (
            "unsupported-intel-gpu",
            indoc! {
                "You seem to be using an Intel GPU that is not supported on Windows 10. \
                You will need to install an older version of Java, [see here for help](https://github.com/MultiMC/MultiMC5/wiki/Unsupported-Intel-GPUs)"
            }
        ),
        (
            "32-it-java",
            indoc! {
                "You're using 32-bit Java. [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)"
            }
        )
    ]);
}
