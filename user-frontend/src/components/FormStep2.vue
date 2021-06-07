<template>
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
    </el-form-item>
  </el-form>
</template>

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
    },
  },
};
</script>