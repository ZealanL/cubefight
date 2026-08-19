package zealan.cubefight.mixins;

import net.minecraft.client.world.ClientWorld;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import static zealan.cubefight.Globals.EVENT_HANDLER;

@Mixin(ClientWorld.class)
public class MixinClientWorld {
    @Inject(method = "tick", at = @At("TAIL"))
    public void tickStart(CallbackInfo ci) {
        EVENT_HANDLER.onTickStart();
    }

    @Inject(method="tick", at=@At("TAIL"))
    public void tickEnd(CallbackInfo ci) {
        EVENT_HANDLER.onTickEnd();
    }
}
