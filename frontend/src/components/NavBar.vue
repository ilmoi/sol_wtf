<template>
  <div>
    <nav class="nav-bar">
      <NavButton
          v-for="item in public"
          :id="item.url"
          :url="item.url"
          :title="item.title"
          :class="{active: currentPage === item.url}"
      />
    </nav>
  </div>
</template>


<script>
import NavButton from "@/components/NavButton";
export default {
  components: {NavButton},
  data() {
    return {
      public: [
        {
          title: "feed",
          url: "/",
        },
        {
          title: "about",
          url: "/about",
        },
        {
          title: "tip",
          url: "/tip",
        },
      ],
    }
  },
  computed: {
    currentPage() {
      return this.$route.path
    },
  },
}
</script>


<style scoped>
.nav-bar {
  @apply flex justify-center items-center m-auto
  bg-solana-purple dark:bg-solana-verydarkgreen;
  /*border border-solid border-solana-purple dark:border-solana-green;*/
  /*min-width: 1200px;*/
}

</style>