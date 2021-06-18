<template>
  <div>
    <NavBar/>
    <div>
      <router-view/>
    </div>
  </div>
</template>

<script>
import NavBar from "@/components/NavBar";
export default {
  components: {NavBar}
}
</script>

<style>
@font-face {
  font-family: "atari";
  src: local("atari"), url('./assets/Atari.ttf') format("truetype");
}
</style>