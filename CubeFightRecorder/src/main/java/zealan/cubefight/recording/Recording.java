package zealan.cubefight.recording;

import com.google.gson.*;
import net.minecraft.block.Blocks;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Box;
import net.minecraft.util.math.Vec3d;

import java.io.IOException;
import java.lang.reflect.Type;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashSet;

import static zealan.cubefight.Globals.MC;

public class Recording {
    private final ArrayList<TickRecord> ticks = new ArrayList<>();
    private final HashSet<BlockInfo> blocks_involved = new HashSet<>();

    public void recordTick() {
        {
            Box hitbox = MC.player.getBoundingBox().expand(0.25, 0.25, 0.25);
            BlockPos minBlockPos = new BlockPos(hitbox.minX, hitbox.minY, hitbox.minZ);
            BlockPos maxBlockPos = new BlockPos(hitbox.maxX, hitbox.maxY, hitbox.maxZ);

            for (int x = minBlockPos.getX(); x <= maxBlockPos.getX(); x++) {
                for (int y = minBlockPos.getY(); y <= maxBlockPos.getY(); y++) {
                    for (int z =  minBlockPos.getZ(); z <= maxBlockPos.getZ(); z++) {
                        BlockPos blockPos = new BlockPos(x, y, z);
                        if (MC.world.getBlockState(blockPos).getBlock() != Blocks.AIR) {
                            this.blocks_involved.add(new BlockInfo(blockPos));
                        }
                    }
                }
            }
        }

        ticks.add(new TickRecord(
                new EntityRecord(MC.player),
                new ControlsRecord(MC.options)
        ));
    }

    public int size() {
        return ticks.size();
    }

    public int numBlocksInvolved() {
        return blocks_involved.size();
    }

    public static class Vec3dSerializer implements JsonSerializer<Vec3d> {
        @Override
        public JsonElement serialize(Vec3d src, Type typeOfSrc, JsonSerializationContext context) {
            JsonArray array = new JsonArray();

            array.add(new JsonPrimitive(src.x));
            array.add(new JsonPrimitive(src.y));
            array.add(new JsonPrimitive(src.z));

            return array;
        }
    }

    public static class BlockInfoSerializer implements JsonSerializer<BlockInfo> {
        @Override
        public JsonElement serialize(BlockInfo src, Type typeOfSrc, JsonSerializationContext context) {
            JsonArray array = new JsonArray();

            array.add(new JsonPrimitive(src.x));
            array.add(new JsonPrimitive(src.y));
            array.add(new JsonPrimitive(src.z));

            return array;
        }
    }

    public void saveTo(String fileName) {
        Gson gson = new GsonBuilder()
                .registerTypeAdapter(Vec3d.class, new Vec3dSerializer())
                .registerTypeAdapter(BlockInfo.class, new BlockInfoSerializer())
                .setPrettyPrinting()
                .create();
        String jsonStr = gson.toJson(this);
        try {
            Files.write(Paths.get(fileName), jsonStr.getBytes(StandardCharsets.UTF_8));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
