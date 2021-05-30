<template>
<el-row type="flex" justify="center">
  <el-col :xs="4" :sm="6" :md="4" :lg="4" :xl="4">
    <LoginForm />
  </el-col>
</el-row>
</template>


<script>
export default {
    name: "login",
    components: {
        LoginForm: () => import("../components/LoginForm.vue"),
    },
};
</script>
