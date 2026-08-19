package zealan.cubefight.recording;

import net.minecraft.entity.Entity;
import net.minecraft.util.math.Vec3d;

public class EntityRecord {
    final Vec3d pos;
    final Vec3d vel;
    final double yaw;
    final double pitch;
    final boolean on_ground;
    final boolean sprinting;
    final boolean sneaking;

    public EntityRecord(Entity entity) {
        this.pos = entity.getPos();
        this.vel = new Vec3d(entity.velocityX, entity.velocityY, entity.velocityZ);
        this.yaw = entity.yaw;
        this.pitch = entity.pitch;
        this.on_ground = entity.onGround;
        this.sprinting = entity.isSprinting();
        this.sneaking = entity.isSneaking();
    }
}
