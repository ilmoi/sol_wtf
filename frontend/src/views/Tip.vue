<template>
  <div class="flex flex-col items-center justify-center pt-2">
    <Box>
      <p>If you use sol.wtf and find it valuable, feel free to buy me a beer.</p>
      <p>My SOL/COPE address is: <a class="text-solana-purple dark:text-solana-pink">5u1vB9UeQSCzzwEhmKPhmQH1veWP9KZyZ8xFxFrmj8CK</a></p>
      <p>And in case you're old school...</p>
      <p>ETH: <a class="text-solana-purple dark:text-solana-pink">0xBDEc2C40857B049067dcd08C094a26eA3997Ca35</a></p>
      <p>BTC: <a class="text-solana-purple dark:text-solana-pink">3QvUQRUgNiVjD63Ny9oA18Ndeth7tk6jVW</a></p>
      <p>Huge thanks for your support. ❤️️</p>
      <p>May our bags grow together...</p>
      <img src="../assets/grateful_pepe.png" class="pepe">
    </Box>
  </div>
</template>

<script>
import Box from "@/components/Box";
export default {
  name: "Tip",
  components: {Box}
}
</script>

<style scoped>
.pepe {
  max-width: 200px;
}
</style>