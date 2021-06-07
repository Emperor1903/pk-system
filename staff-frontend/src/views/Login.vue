<template>
<el-row type="flex" justify="center">
  <el-col style="max-width: 300px">
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
