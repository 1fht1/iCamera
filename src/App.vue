<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted} from "vue";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";

onMounted(() => {
  invoke("reload")
  show()
})
await listen<string>("videoSelect", (event) => {
  show(event.payload)
})
await listen<string>("reload", () => {
  navigator.mediaDevices.enumerateDevices().then(devices => {
    invoke("update_video_menu", {
      videoList: JSON.stringify(devices.filter(device => device.kind === "videoinput").map(device => (
        {label: device.label, deviceId: device.deviceId}
      )))
    })
  })
})

async function show(deviceId: string = "") {
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
  <div data-tauri-drag-region style="height: 100%;width: 100%;overflow: hidden">
    <video style="height: 100%;width: 100%;border-radius: 10px"/>
  </div>
</template>

<style scoped>

</style>
