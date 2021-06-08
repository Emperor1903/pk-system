<template>
<el-dialog
  v-loading="loading"
  :title="state.title"
  :visible.sync="state.visible"
  width="400px">
  <el-form :model="form" :rules="rules" ref="form" status-icon>
    <el-form-item label="Tên tài khoản" :label-width="formLabelWidth" prop="username">
      <el-input v-model="form.username" autocomplete="off"></el-input>
    </el-form-item>
    <el-form-item label="Mật khẩu" :label-width="formLabelWidth" prop="password">
      <el-input v-model="form.password" autocomplete="off"  show-password></el-input>
    </el-form-item>
    
  </el-form>
  <span slot="footer" class="dialog-footer">
    <el-button @click="handleCancel">Thoát</el-button>
    <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
  </span>
</el-dialog>
</template>

<script>
import { newAdmin } from "../api/index.js";
export default {
    name: "NewAdminDialog",
    props: ["state"],
    data() {
        var validateUser = (rule, value, callback) => {
            if (value.length < 5) callback(new Error("Độ dài tên tài khoản phải lớn 5"))
            else callback();
        }
        var validatePass = (rule, value, callback) => {
            if (value === '') callback(new Error("Hãy nhập mật khẩu"));
            else if (value.length < 5) callback(new Error("Độ dài mật khẩu phải lớn 5"));
            else callback();
        }
        
        return {
            form: {
                username: "",
                password: "",
            },
            rules: {
                username: [
                    { validator: validateUser, trigger: "blur" }
                ],
                password: [
                    { validator: validatePass, trigger: "blur" }
                ],
            },
            loading: false,
            formLabelWidth: "120px",
        };
    },
    methods: {
        async handleConfirm() {
            await this.$refs["form"].validate(async (valid) => {
                console.log(valid);
                if (valid) {
                    this.loading = true;
                    await newAdmin(
                        this.form.username,
                        this.form.password
                    );
                    this.loading = false;
                    this.state.visible = false;
                    this.$emit("confirm");        
                } else {
                    return false;
                }
            });
            
            
        },
        async handleCancel() {
            this.$refs["form"].resetFields();
            this.state.visible = false;
        },
    },
};
</script>
