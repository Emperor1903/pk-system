<template>
<div class="home">
  <HomeBody/>
</div>
</template>

<script>
  
export default {
    name: 'Home',
    components: {
        HomeBody: () => import("../components/HomeBody.vue"),
    }
}
</script>
