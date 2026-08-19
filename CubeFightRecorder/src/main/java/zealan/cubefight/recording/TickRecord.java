package zealan.cubefight.recording;

public class TickRecord {
    public final EntityRecord player;
    public final ControlsRecord controls;

    public TickRecord(EntityRecord player, ControlsRecord controls) {
        this.player = player;
        this.controls = controls;
    }
}
