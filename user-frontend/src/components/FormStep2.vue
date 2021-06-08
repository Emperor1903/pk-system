<template>
<<<<<<< HEAD
  <el-form label-width="120px">
    <el-form-item label="Tỉnh, Thành phố">
      <el-autocomplete
        :fetch-suggestions="handleProvinceQuery"
        @select="handleProvinceSelect"
        v-model="provinceName"
        class="select" />
      
    </el-form-item>
    <el-form-item label="Bệnh viện">
      <el-autocomplete
        :disabled="!form.province"
        :fetch-suggestions="handleHospitalQuery"
        @select="handleHospitalSelect"
        v-model="hospitalName"        
        class="select">
      </el-autocomplete>
    </el-form-item>
    <el-form-item label="Phòng khám">
      <el-autocomplete
        :disabled="!form.province"
        :fetch-suggestions="handleClinicQuery"
        @select="handleClinicSelect"
        v-model="clinicName"        
        class="select">
      </el-autocomplete>
=======
  <el-form>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.province" @change="getHospital">
        <el-option
          v-for="item in provinceData"
          :value="item._id"
          :label="item.name"
          :key="item._id"
        >
        </el-option>
      </el-select>
    </el-form-item>
    <el-form-item label="Chọn bệnh viện">
      <el-select v-model="form.hospital" @change="getClinic">
        <el-option
          v-for="item in hospitalData"
          :value="item._id"
          :label="item.name"
          :key="item._id"
        >
        </el-option>
      </el-select>
    </el-form-item>
    <el-form-item label="Chọn khoa">
      <el-select v-model="form.clinic" @change="getDoctor">
        <el-option
          v-for="item in clinicData"
          :value="item._id"
          :label="item.name"
          :key="item._id"
        >
        </el-option>
      </el-select>
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
    </el-form-item>
  </el-form>
</template>

<<<<<<< HEAD
<style>
.select {
    min-width: 500px;
}
</style>

<script>
import {searchProvince, searchHospital, searchClinic} from "../api/index"

export default {
    props: ["form"],
    data() {
        return {
            provinceName: "",
            hospitalName: "",
            clinicName: "",
        };
    },
    async mounted() {
        
=======
<script>
import { searchProvince, searchHospital, searchClinic } from "../api/api";

export default {
  props: ["form"],
  data() {
    return {
      provinceData: [],
      hospitalData: [],
      clinicData: [],
    };
  },
  async mounted() {
    await this.getProvince();
  },
  methods: {
    async getProvince() {
      let data = await searchProvince();
      this.provinceData = data.data;
    },
    async getHospital() {
      this.hospitalData = [];
      this.clinicData = [];
      let data = searchHospital(form.province);
      this.hospitalData = data.data;
    },
    async getClinic() {
      this.clinicData = [];
      let data = searchClinic(form.hospital);
      this.clinicData = data.data;
    },
    async getDoctor() {
      this.form.doctorData = [];
      let data = searchDoctor(this.form.clinic);
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
    },
    methods: {
        async handleProvinceQuery(keyword, callback) {
            var data = await searchProvince(keyword);
            if (data) {
                var rs = data.data.map( i => {
                    return {
                        value: i.name,
                        id: i._id,
                    }
                });
                callback(rs);
            }
        },
        handleProvinceSelect(item) {
            this.form.province = item.id;
        },
        async handleHospitalQuery(keyword, callback) {
            var data = await searchHospital(keyword, this.form.province, 0);
            if (data) {
                var rs = data.data.map( i => {
                    return {
                        value: i.name,
                        id: i._id,
                    }
                });
                callback(rs);
            }
        },
        handleHospitalSelect(item) {
            this.form.hospital = item.id;
        },
        async handleClinicQuery(keyword, callback) {
            var data = await searchClinic(keyword,
                                          this.form.hospital, 0);
            if (data) {
                var rs = data.data.map( i => {
                    return {
                        value: i.name,
                        id: i._id,
                    }
                });
                callback(rs);
            }
        },
        handleClinicSelect(item) {
            this.form.clinic = item.id;
        },
  },
};
</script>
