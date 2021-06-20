<template>
  <div>
    <div class="img-wrapper" v-if="showDetails">
      <slot/>
    </div>
    <button
        class="details-btn"
        :class="textColor"
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
    darkBg: Boolean,
  },
  data() {
    return {
      showDetails: false
    }
  },
  computed: {
    textColor() {
      if (this.darkBg) {
        return !this.showDetails ? 'text-white dark:text-black' : 'text-gray-300 dark:text-gray-600'
      } else {
        return !this.showDetails ? 'text-black dark:text-white' : 'text-gray-500 dark:text-gray-400'
      }
    }
  }
}
</script>

<style scoped>
.details-btn {
  @apply m-2 underline;
  outline: none;
}

.img-wrapper {
  @apply border-2 border-solid border-black p-4 mt-2 text-left;
  border-right-color: white;
  border-bottom-color: white;
}
</style>