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
  @apply px-10 py-1 text-white flex-shrink-0;
    /*border border-solid border-solana-purple dark:border-solana-verydarkgreen;*/
  outline: none;
}

.nav-btn:hover {
  @apply bg-solana-darkpurple dark:bg-solana-darkgreen text-white dark:text-white;
  /*border border-solid border-solana-verydarkpurple dark:border-solana-green;*/
}

.active {
  @apply bg-solana-darkpurple dark:bg-solana-darkgreen text-white dark:text-white;
  /*border border-solid border-solana-verydarkpurple dark:border-solana-green;*/
}
</style>