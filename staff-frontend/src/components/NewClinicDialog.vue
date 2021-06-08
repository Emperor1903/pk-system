<template>
<el-dialog
  v-loading="loading"
  :title="state.title"
  :visible.sync="state.visible"
  width="50%">
  <el-form :model="form">
    <el-form-item label="Tên phòng khám" :label-width="formLabelWidth">
      <el-input v-model="form.name" autocomplete="off"></el-input>
    </el-form-item>
    <el-form-item label="Mô tả" :label-width="formLabelWidth">
      <el-input
        type="textarea"
        :autosize="{ minRows: 4, maxRows: 4}"
        placeholder="Nhập mô tả phòng khám"
        v-model="form.desc">
      </el-input>
    </el-form-item>
    <el-form-item label="Thời gian mở cửa" :label-width="formLabelWidth">
      <el-input
        type="textarea"
        :autosize="{ minRows: 2, maxRows: 2}"
        placeholder="Thời gian mở cửa của phòng khám"
        v-model="form.timeDesc">
      </el-input>
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
    <el-form-item label="Bệnh viện" :label-width="formLabelWidth">
      <el-autocomplete
        v-model="hospitalInputState"
        :fetch-suggestions="handleHospitalQuery"
        @select="handleHospitalSelect"
        placeholder="Tìm kiếm">
      </el-autocomplete>
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
import {newClinic, uploadImage, searchHospital} from "../api/index.js";

export default {
    name: "NewClinicDialog",
    props: ["state"],
    data() {
        return {
            form: {
                name: "",
                desc: "",
                timeDesc: "",
                address: "",
                phoneNum: "",
                hospital: null,
                hospitalName: "",
                images: [],
            },
            hospitalInputState: "",
            loading: false,
            formLabelWidth: "120px",
        };
    },
    watch: {
        state: function(val) {
            this.hospitalInputState = val.hospitalName;
            console.log(this.hospitalInputState);
        }
    },
    methods: {
        async handleConfirm() {
            this.loading = true;
            
            if (!this.form.hospital)
                this.form.hoaspital = this.state.hospital;
            
            await newClinic(
                this.form.name,
                this.form.desc,
                this.form.address,                
                this.form.timeDesc,
                this.form.phoneNum,
                this.form.hospital,
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
        },
        async handleHospitalQuery(keyword, callback) {
            var response = await searchHospital(keyword, 0, 1000);
            let data = response.data.map(i => {
                return {
                    value: i.name,
                    hospital: i._id
                }
            });
            callback(data);
        },
        handleHospitalSelect(item) {
            this.form.hospital = item.hospital;
        }
    }
}
</script>
