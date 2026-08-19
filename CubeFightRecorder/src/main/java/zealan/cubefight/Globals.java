package zealan.cubefight;

import net.minecraft.client.MinecraftClient;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import zealan.cubefight.recording.Recording;

public class Globals {
    private static boolean initialized = false;

    public static final String MOD_ID = "cubefightrecorder";
    public static final Logger MLOGGER = LogManager.getLogger(MOD_ID);

    public static EventHandler EVENT_HANDLER;
    public static MinecraftClient MC;

    public static boolean IS_RECORDING = false;
    public static Recording CUR_RECORDING = null;

    public static void init(CubeFightRecorderMod modInst) {
        if (initialized)
            throw new RuntimeException("Already initialized");
        initialized = true;

        EVENT_HANDLER = new EventHandler();
        MC = MinecraftClient.getInstance();
    }
}
