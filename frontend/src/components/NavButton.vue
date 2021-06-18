<template>
  <router-link :to="url" class="nav-btn">
    {{title}}
  </router-link>
</template>


<script>
export default {
  props: {
    title: String,
    url: String,
  }
}
</script>


<style scoped>
.nav-btn {
  @apply px-10 py-1 text-black
  hover:bg-blue-500 hover:text-white flex-shrink-0;
  outline: none;
}
.active {
  @apply bg-blue-500 text-white;
}
</style>