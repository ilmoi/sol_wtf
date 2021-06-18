<template>
  <!--extra div to control behavior on ultra-narrow screens (min 1000px)-->
  <div class="bg-blue-300">
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
  bg-red-200;
  /*min-width: 1200px;*/
}

</style>