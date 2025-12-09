<template>
  <div
    :class="
      cn(
        'relative size-full overflow-hidden bg-[radial-gradient(ellipse_at_bottom,#262626_0%,#000_100%)]',
        props.class,
      )
    "
  >
    <!-- Star Layer 1 (small, fast) -->
    <div class="stars-layer-1">
      <div class="stars stars-1" :style="{ boxShadow: boxShadow1 }" />
    </div>

    <!-- Star Layer 2 (medium, moderate speed) -->
    <div class="stars-layer-2">
      <div class="stars stars-2" :style="{ boxShadow: boxShadow2 }" />
    </div>

    <!-- Star Layer 3 (large, slow) -->
    <div class="stars-layer-3">
      <div class="stars stars-3" :style="{ boxShadow: boxShadow3 }" />
    </div>

    <!-- Slot for child content -->
    <slot />
  </div>
</template>

<script setup lang="ts">
import { cn } from "@/lib/utils";
import { onMounted, ref, watch } from "vue";

interface StarsBackgroundProps {
  starColor?: string;
  class?: string;
}

const props = withDefaults(defineProps<StarsBackgroundProps>(), {
  starColor: "#fff",
});

// For slot content
defineSlots();

function generateStars(count: number, starColor: string) {
  const shadows: string[] = [];
  for (let i = 0; i < count; i++) {
    const x = Math.floor(Math.random() * 2000);
    const y = Math.floor(Math.random() * 2000);
    shadows.push(`${x}px ${y}px ${starColor}`);
  }
  return shadows.join(", ");
}

const boxShadow1 = ref("");
const boxShadow2 = ref("");
const boxShadow3 = ref("");

onMounted(() => {
  boxShadow1.value = generateStars(700, props.starColor);
  boxShadow2.value = generateStars(200, props.starColor);
  boxShadow3.value = generateStars(100, props.starColor);
});

// Watch for starColor changes
watch(
  () => props.starColor,
  (newColor) => {
    boxShadow1.value = generateStars(700, newColor);
    boxShadow2.value = generateStars(200, newColor);
    boxShadow3.value = generateStars(100, newColor);
  },
);
</script>

<style scoped>
/* Base styles for all star layers */
.stars {
  position: absolute;
  top: 0;
  left: 0;
  background: transparent;
  border-radius: 50%;
  will-change: transform;
}

/* Layer containers with parallax offset */
.stars-layer-1,
.stars-layer-2,
.stars-layer-3 {
  position: absolute;
  inset: 0;
  transition: transform 4s cubic-bezier(0.16, 1, 0.3, 1);
}

.stars-layer-1 {
  transform: translate(
    calc(var(--mouse-x, 0px) * 0.3),
    calc(var(--mouse-y, 0px) * 0.3)
  );
}

.stars-layer-2 {
  transform: translate(
    calc(var(--mouse-x, 0px) * 0.6),
    calc(var(--mouse-y, 0px) * 0.6)
  );
}

.stars-layer-3 {
  transform: translate(var(--mouse-x, 0px), var(--mouse-y, 0px));
}

/* Star animations (vertical scrolling) */
.stars-1 {
  width: 1px;
  height: 1px;
  animation: animStar1 50s linear infinite;
}

.stars-1::after {
  content: " ";
  position: absolute;
  top: 2000px;
  left: 0;
  width: 1px;
  height: 1px;
  background: transparent;
  border-radius: 50%;
  box-shadow: inherit;
}

.stars-2 {
  width: 2px;
  height: 2px;
  animation: animStar2 100s linear infinite;
}

.stars-2::after {
  content: " ";
  position: absolute;
  top: 2000px;
  left: 0;
  width: 2px;
  height: 2px;
  background: transparent;
  border-radius: 50%;
  box-shadow: inherit;
}

.stars-3 {
  width: 3px;
  height: 3px;
  animation: animStar3 150s linear infinite;
}

.stars-3::after {
  content: " ";
  position: absolute;
  top: 2000px;
  left: 0;
  width: 3px;
  height: 3px;
  background: transparent;
  border-radius: 50%;
  box-shadow: inherit;
}

/* Animations - separate for each layer to avoid conflicts */
@keyframes animStar1 {
  from {
    transform: translateY(0px);
  }
  to {
    transform: translateY(-2000px);
  }
}

@keyframes animStar2 {
  from {
    transform: translateY(0px);
  }
  to {
    transform: translateY(-2000px);
  }
}

@keyframes animStar3 {
  from {
    transform: translateY(0px);
  }
  to {
    transform: translateY(-2000px);
  }
}
</style>
