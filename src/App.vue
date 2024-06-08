<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted, ref} from "vue";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api";
import {getCurrent, LogicalSize} from "@tauri-apps/api/window";

const windowSizeConfigSets = {
  "large": {
    "label": "大",
    "width": 640,
    "height": 360
  },
  "medium": {
    "label": "中",
    "width": 480,
    "height": 270
  },
  "small": {
    "label": "小",
    "width": 320,
    "height": 180
  },
  "tiny": {
    "label": "迷你",
    "width": 160,
    "height": 90
  }
}

onMounted(() => {
  invoke("reload")
  show()
})
listen<string>("videoSelect", (event) => {
  show(event.payload)
})
listen<string>("reload", () => {
  navigator.mediaDevices.enumerateDevices().then(devices => {
    invoke("update_video_menu", {
      videoList: devices.filter(device => device.kind === "videoinput").map(device => (
        {label: device.label, device_id: device.deviceId}
      ))
    })
  })
})

let shape = ref("rectangle")
let windowSize = ref<keyof typeof windowSizeConfigSets>("large")

listen<string>("shape", () => {
  if (shape.value === "rectangle") {
    shape.value = "circle"
    getCurrent().setSize(new LogicalSize(windowSizeConfigSets[windowSize.value].height, windowSizeConfigSets[windowSize.value].height))
  } else {
    shape.value = "rectangle"
    getCurrent().setSize(new LogicalSize(windowSizeConfigSets[windowSize.value].width, windowSizeConfigSets[windowSize.value].height))
  }
})

listen<string>("windowSize", (event) => {
  windowSize.value = event.payload as keyof typeof windowSizeConfigSets
  if (shape.value === "rectangle") {
    getCurrent().setSize(new LogicalSize(windowSizeConfigSets[windowSize.value].width, windowSizeConfigSets[windowSize.value].height))
  } else {
    getCurrent().setSize(new LogicalSize(windowSizeConfigSets[windowSize.value].height, windowSizeConfigSets[windowSize.value].height))
  }
})

function show(deviceId: string = "") {
  const video = document.querySelector('video')!;
  navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      deviceId: deviceId === "" ? undefined : deviceId,
      width: 1920,
      height: 1080,
    }
  }).then(stream => {
    video.srcObject = stream;
    video.onloadedmetadata = () => {
      video.play();
    }
  }).catch(err => {
    console.log("调用摄像头失败:", +err.name + ':' + err.message)
  })
}
</script>

<template>
  <div data-tauri-drag-region style="overflow: hidden;"
       :class="shape==='rectangle'?'rectangle':windowSize">
    <video style="height: 100%;width: 100%;object-fit: cover;pointer-events: none;"
           :style="shape==='rectangle'?'border-radius: 0;':'border-radius: 50%;'"/>
  </div>
</template>

<style scoped>
body, html {
  background-color: transparent; /* 确保背景透明 */
}

.rectangle {
  width: 100vw;
  height: 100vh;
}

.large {
  width: 360px;
  height: 360px;
  border-radius: 50%; /* 创造圆形效果 */
}

.medium {
  width: 270px;
  height: 270px;
  border-radius: 50%;
}

.small {
  width: 180px;
  height: 180px;
  border-radius: 50%;
}

.tiny {
  width: 90px;
  height: 90px;
  border-radius: 50%;
}
</style>
