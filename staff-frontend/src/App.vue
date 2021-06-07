<template>
<div id="app">
  <Admin v-if="isAdmin"/>
  <Login v-if="notLogin" />
</div>
</template>

<script>
import { getIdentity } from "./api/auth";

export default {
    components: {
        Admin: () => import("./views/Admin.vue"),
        Login: () => import("./views/Login.vue"),        
    },
    data() {
        return {
            identity: -1,
        }
    },
    computed: {
        isAdmin() {
            return this.identity == 0;
        },
        isStaff() {
            return this.identity == 1;
        },
        notLogin() {
            return this.identity < 0;
        },
    },
    async mounted() {
        var identity = await getIdentity();
        if (identity) this.identity = identity.role;
    }
};
</script>

