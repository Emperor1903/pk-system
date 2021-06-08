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
  </el-row>
  <el-row type="flex" justify="center">
    <el-col>
      <div style="padding: 14px; max-width: 800px; margin: auto" class="para">
        <span> <strong>Tên bác sĩ:</strong> {{ this.data.name }}</span> <br/>
        <span> <strong>Vị trí:</strong> {{ this.data.position }}</span> <br/>
        <span> <strong>Số điện thoại:</strong> {{ this.data.phone_num }}</span> <br/>
        <span> <strong>Email:</strong> {{ this.data.email }}</span> <br/>
        <span> <strong>Giới thiệu ngắn:</strong> {{ this.data.short_intro }}</span> <br/>
        <span> <strong>Giới thiệu:</strong> {{ this.data.intro }}</span> <br/>
        <span> <strong>Phòng khám:</strong> {{ this.data.clinicName }}</span> <br/>
        <span> <strong>Chuyên môn:</strong> {{ this.data.specializationName }}</span> <br/>
        <hr>
        <el-row type="flex" class="row-bg" justify="center" >
          <el-button icon="el-icon-edit" type="primary" @click="handleEdit">
            Chỉnh sửa
          </el-button>
        </el-row>
      </div>      
    </el-col>
  </el-row>
  
  <UpdateDoctorDialog :state="editState" @confirm="updateData"/>    
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
                doc: "doctor",
                visible: false,
            },
        };
    },
    components: {
        UpdateDoctorDialog: () => import("./UpdateDoctorDialog.vue"),
    },
    async mounted() {
        await this.getData();
    },
    methods: {
        async getData() {
            var id = {"$oid": this.id};
            var data = await getDocument("doctor", id);

            var r1 = await getDocument("specialization", data.specialization);
            var r2 = await getDocument("clinic", data.clinic);
            
            this.data = data;
            this.data.specializationName = r1.name;
            this.data.clinicName = r2.name;
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
