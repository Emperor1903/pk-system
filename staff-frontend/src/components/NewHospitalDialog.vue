<template>
<el-dialog
  v-loading="loading"
  :title="state.title"
  :visible.sync="state.visible"
  width="50%">
  <el-form :model="form">
    <el-form-item label="Tên bệnh viện" :label-width="formLabelWidth">
      <el-input v-model="form.name" autocomplete="off"></el-input>
    </el-form-item>
    <el-form-item label="Mô tả" :label-width="formLabelWidth">
      <el-input
        type="textarea"
        :autosize="{ minRows: 4, maxRows: 4}"
        placeholder="Nhập mô tả bệnh viện"
        v-model="form.desc">
      </el-input>
    </el-form-item>
    <el-form-item label="Tỉnh thành" :label-width="formLabelWidth">
      <el-select v-model="provinceIndex" clearable placeholder="Lựa chọn tỉnh thành">
        <el-option v-for="(item, index) in state.provinces"
                   :key="index"
                   :label="item.name"
                   :value="index">
        </el-option>
      </el-select>
    </el-form-item>
    <el-form-item label="Địa chỉ" :label-width="formLabelWidth">
      <el-input
        type="textarea"
        :autosize="{ minRows: 2, maxRows: 2}"
        placeholder="Nhập địa chỉ của bệnh viện"
        v-model="form.address">
      </el-input>
    </el-form-item>
    <el-form-item label="Số điện thoại" :label-width="formLabelWidth">
      <el-input
        v-model="form.phoneNum" autocomplete="off">
      </el-input>
    </el-form-item>
    <el-form-item label="Hình ảnh" :label-width="formLabelWidth">
      <el-upload
        ref="upload"
        action=""
        :limit="3"
        :auto-upload="false"
        :http-request="handleImageUpload"
        :file-list="form.images">
        <el-button size="small" slot="trigger" type="primary">Đăng ảnh</el-button>
        <el-button size="small" style="margin-left: 10px;" type="success" @click="$refs.upload.submit()">
          Tải ảnh lên server
        </el-button>        
      </el-upload>

    </el-form-item>
  </el-form>
  
  <span slot="footer" class="dialog-footer">
    <el-button @click="handleCancel">Thoát</el-button>
    <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
  </span>
</el-dialog>
</template>

<script>
import {newHospital, uploadImage} from "../api/admin.js";

export default {
    name: "NewHospitalDialog",
    props: ["state"],
    data() {
        return {
            form: {
                name: "",
                desc: "",
                province: {},
                address: "",
                phoneNum: "",
                images: [],
            },
            loading: false,
            provinceIndex: null,
            formLabelWidth: "120px",
        };
    },
    watch: {
        provinceIndex: function(val){
            this.form.province = this.state.provinces[val]._id;
        }
    },
    methods: {
        async handleConfirm() {
            this.loading = true;
            await newHospital(
                this.form.name,
                this.form.desc,
                this.form.province,
                this.form.address,
                this.form.phoneNum,
                this.form.images.map(i => i.url)
            );
            this.loading = false;            
            this.state.visible = false;
            this.$emit('confirm');            
        },
        async handleCancel() {
            this.state.visible = false;
        },
        async handleImageUpload(params) {
            try {
                this.loading = true;
                var response = await uploadImage(params.file);
                if (response.status == 200) {
                    var data = response.data;
                    var i = data.display_url
                    this.form.images.push({
                        name: i.split("/")[i.split("/").length - 1],
                        url: i
                    });                    
                } else {
                }
            } finally {
                this.loading = false;
            }
        }
    }
}
</script>
