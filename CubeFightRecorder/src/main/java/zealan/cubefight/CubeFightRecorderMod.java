package zealan.cubefight;

import net.fabricmc.api.ModInitializer;
import net.legacyfabric.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.legacyfabric.fabric.api.client.keybinding.v1.KeyBindingHelper;
import net.legacyfabric.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.font.TextRenderer;
import net.minecraft.client.option.KeyBinding;
import net.minecraft.text.LiteralText;
import org.lwjgl.input.Keyboard;

import static zealan.cubefight.Globals.EVENT_HANDLER;
import static zealan.cubefight.Globals.MC;

public class CubeFightRecorderMod implements ModInitializer {
    private static KeyBinding toggleKey;

    @Override
    public void onInitialize() {
        Globals.init(this);

        toggleKey = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key.cubefightrecorder.toggle",
                Keyboard.KEY_R,
                "category.cubefightrecorder"
        ));

        ClientTickEvents.END_CLIENT_TICK.register(client -> {
            if (MC.player != null && toggleKey.wasPressed())
                EVENT_HANDLER.onToggled();
        });

        HudRenderCallback.EVENT.register((mc, partialTicks) -> {
            MinecraftClient client = MinecraftClient.getInstance();

            if (client.player != null && !client.options.debugEnabled)
                EVENT_HANDLER.onRenderHud();
        });
    }
}