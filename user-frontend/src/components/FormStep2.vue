<template>
  <el-form>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.province"> </el-select>
    </el-form-item>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.hospital"> </el-select>
    </el-form-item>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.clinic"> </el-select>
    </el-form-item>
  </el-form>
</template>

<script>

import {searchProvince} from "../api/api"

export default {
  props: ["form"],
  data() {
    return {
      provinceData: [],
    };
  },
  async mounted() {
    await this.getProvince();
  },
  methods: {
    async getProvince() {
      let data = await searchProvince();
      console.log(data);
    },
  },
};
</script>