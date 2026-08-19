package zealan.cubefight.recording;

import net.minecraft.util.math.BlockPos;

import java.util.Objects;

public class BlockInfo {
    final int x, y, z;

    public BlockInfo(BlockPos pos) {
        this.x = pos.getX();
        this.y = pos.getY();
        this.z = pos.getZ();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        BlockInfo other = (BlockInfo) o;
        return (x == other.x) && (y == other.y) && (z == other.z);
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y, z);
    }
}
