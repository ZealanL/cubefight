package zealan.cubefight;

import net.minecraft.client.font.TextRenderer;
import zealan.cubefight.recording.Recording;

import java.util.ArrayList;

import static zealan.cubefight.Globals.*;

public class EventHandler {
    public void onTickStart() {

    }

    public void onTickEnd() {
        if (IS_RECORDING && CUR_RECORDING != null) {
            CUR_RECORDING.recordTick();
        }
    }

    public void onRenderHud() {
        TextRenderer fontRenderer = MC.textRenderer;

        ArrayList<String> lines = new ArrayList<>();
        lines.add("CubeFightRecorder: [" + (IS_RECORDING ? "RECORDING" : "IDLE") + "]");

        if (CUR_RECORDING != null) {
            lines.add("Recording: [Length=" + CUR_RECORDING.size() + ", blocks=" + CUR_RECORDING.numBlocksInvolved() + "]");
        }

        for (int i = 0; i < lines.size(); i++) {
            fontRenderer.drawWithShadow(
                    lines.get(i), 5, 5 + (i * 10), IS_RECORDING ? 0xFF8888 : 0xFFFFFF
            );
        }

    }

    public void onToggled() {
        IS_RECORDING = !IS_RECORDING;
        if (IS_RECORDING) {
            CUR_RECORDING = new Recording();
        } else {
            if (CUR_RECORDING != null) {
                CUR_RECORDING.saveTo("recording.json");
            }
            CUR_RECORDING = null;
        }
    }
}
