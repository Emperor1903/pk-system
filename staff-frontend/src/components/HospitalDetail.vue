<template>
<el-card shadow="never" :body-style="{ padding: '0px'}" >
  <el-row type="flex" justify="center">
    <el-col>
      <el-carousel
        indicator-position="outside"
        :interval=10000>
        <el-carousel-item v-for="(item, index) in data.images" :key="index">
          <img :src="item" class="image"/>
        </el-carousel-item>
      </el-carousel>      
    </el-col>
    <el-col>
      <div style="padding: 14px; max-width: 400px; margin: auto" class="para">
        <span> <strong>Bệnh viện:</strong> {{ this.data.name }}</span> <br/>
        <span> <strong>Số điện thoại:</strong> {{ this.data.phone_num }}</span> <br/>
        <span> <strong>Địa chỉ:</strong> {{ this.data.address }}</span> <br/>
        <span> <strong>Mô tả:</strong> {{ this.data.desc }}</span> <br/>
        <hr>
        <el-row type="flex" class="row-bg" justify="center" >
          <el-button icon="el-icon-edit" type="primary" @click="handleEdit">
            Chỉnh sửa
          </el-button>
        </el-row>
      </div>      
    </el-col>
  </el-row>  
  
  <UpdateHospitalDialog :state="editState" @confirm="updateData"/>    
</el-card>
</template>

<style>
.para {
    line-height: 1.6;
}

.el-carousel__item {
    background-color: #000000;
}

.image {
    height: 100%;
    margin: auto;
    display: block;
}
</style>

<script>
import {getDocument, searchProvince} from "../api/index.js";

export default {
    name: "DeleteDialog",
    props: ["id"],
    data() {
        return {
            data: {
                name: "",
                desc: "",
                address: "",
                province: {},
                phone_num: "",
                images: [],
            },
            editState: {
                title: "Bệnh viện",
                doc: "hospital",
                visible: false,
            },
        };
    },
    components: {
        UpdateHospitalDialog: () => import("./UpdateHospitalDialog.vue"),
    },
    async mounted() {
        await this.getData();
    },
    methods: {
        async getData() {
            var id = {"$oid": this.id};
            var data = await getDocument("hospital", id);
            
            this.data = data;
        },
        async handleEdit() {
            this.editState.visible = true;
            this.editState.id = this.id;
        },
        async updateData() {
            await this.getData();
        }
    }
}
</script>
