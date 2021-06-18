<template>
  <div  class="box adj_width">
    <slot/>
  </div>
</template>

<script>
export default {
  name: "Box"
}
</script>

<style scoped>

.box {
  @apply my-5 p-5
  bg-solana-verylightpurple dark:bg-solana-verydarkgreen
  border border-solid border-solana-purple dark:border-solana-green;
}

</style>