<template>
<div class="LoginForm">
  <el-card>
    <h2>Login</h2>
    <el-form
      class="login-form"
      :model="model"
      :rules="rules"
      ref="form"
      @submit.native.prevent="login"
      >
      <el-form-item prop="username">
        <el-input v-model="model.username" placeholder="Username" prefix-icon="fas fa-user"></el-input>
      </el-form-item>
      <el-form-item prop="password">
        <el-input
          v-model="model.password"
          placeholder="Password"
          type="password"
          prefix-icon="fas fa-lock">
        </el-input>
      </el-form-item>
      <el-form-item>
        <el-button
          :loading="loading"
          class="login-button"
          type="primary"
          native-type="submit"
          block>Đăng nhập</el-button>
      </el-form-item>
      <!-- <a class="forgot-password" href="https://oxfordinformatics.com/">Forgot password ?</a> -->
    </el-form>
  </el-card>
</div>
</template>

<script>
import {authenticate} from "../api/auth";
  
export default {
    name: "LoginForm",
    data() {
        return {
            model: {
                username: "",
                password: ""
            },
            loading: false,
            rules: {
                username: [
                    {
                        required: true,
                        message: "Username is required",
                        trigger: "blur"
                    },
                    {
                        min: 4,
                        message: "Username length should be at least 5 characters",
                        trigger: "blur"
                    }
                ],
                password: [
                    { required: true, message: "Password is required", trigger: "blur" },
                    {
                        min: 5,
                        message: "Password length should be at least 5 characters",
                        trigger: "blur"
                    }
                ]
            }
        };
    },
    methods: {
        async login() {
            let valid = await this.$refs.form.validate();
            if (!valid) {
                return;
            }
            this.loading = true;
            var rs = await authenticate(this.model.username, this.model.password)
            this.loading = false;
            if (rs === "succeed to login") {
                window.location.reload();
            } else {
                this.$message.error("Username or password is invalid");
            }
        }
    }
};
</script>
