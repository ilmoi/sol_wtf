<template>
  <div>
    <div class="img-wrapper" v-if="showDetails">
      <slot/>
    </div>
    <button
        class="details-btn"
        :class="!showDetails ? 'text-green-200' : 'text-gray-200'"
        @click="showDetails = !showDetails"
    >
      {{ showDetails ? '^' : '>' }} {{ title }}
    </button>
  </div>
</template>

<script>
export default {
  props: {
    title: String,
  },
  data() {
    return {
      showDetails: false
    }
  }
}
</script>

<style scoped>
.details-btn {
  @apply m-2;
  outline: none;
}

.img-wrapper {
  @apply border-2 border-solid border-black p-4 mt-2 text-left;
  border-right-color: white;
  border-bottom-color: white;
}
</style>