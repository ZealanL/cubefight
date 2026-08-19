package zealan.cubefight.recording;

import net.minecraft.client.option.GameOptions;

public class ControlsRecord {
    final boolean move_f;
    final boolean move_b;
    final boolean move_r;
    final boolean move_l;

    final boolean jump;
    final boolean sprint;
    final boolean sneak;
    
    public ControlsRecord(GameOptions options) {
        move_f = options.forwardKey.isPressed();
        move_b = options.backKey.isPressed();
        move_r = options.rightKey.isPressed();
        move_l = options.leftKey.isPressed();

        jump = options.jumpKey.isPressed();
        sprint = options.sprintKey.isPressed();
        sneak = options.sneakKey.isPressed();
    }
}
