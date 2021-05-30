<template>
<el-card shadow="never" :body-style="{ padding: '0px'}" >
  <el-row type="flex" justify="center">
    <el-col>
      <el-carousel
        indicator-position="outside"
        :interval=10000>
        <el-carousel-item v-for="(item, index) in data.images" :key="index">
          <img :src="item.url" class="image"/>
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
import {getDocument, searchProvince} from "../api/admin.js";

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
                data: {},
                provinces: [],
                confirmed: false,
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
            if (data.images) {
                data.images = data.images.map(i => {
                    return {
                        name: i.split("/")[i.split("/").length - 1],
                        url: i
                    }});
            }
            var provinces = await searchProvince();
            
            this.data = data;
            this.editState.provinces = provinces.data;
        },
        async handleEdit() {
            this.editState = {
                title: "Bệnh viện",
                doc: "hospital",
                data: this.data,
                visible: true,
                confirmed: false,
                provinces: this.editState.provinces,
            };
        },
        async updateData() {
            await this.getData();
        }
    }
}
</script>
