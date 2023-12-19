<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function test() {
  invoke("start_edge", {
    config: {
      supernode_addr: "supernode.maa-org.net:7654",
      community: "chingc-home-network",
      username: "ChingCs-MBP",
      password: "990123",
      encryption_key: "ChingCdesu",
    },
  });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <button @click="test">Test</button>
</template>
