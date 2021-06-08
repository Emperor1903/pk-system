<template>
<<<<<<< HEAD
<el-form label-width="120px">
    <el-form-item label="Lựa chọn bác sĩ">
      <el-autocomplete
        :fetch-suggestions="handleDoctorQuery"
        @select="handleDoctorSelect"
        v-model="doctorName"
        class="select" />
=======
  <el-form>
    <el-form-item label="Bác sĩ khám">
      <el-select v-model="form.doctor" @change="getShift">
        <el-option
          v-for="(item, index) in form.doctorData"
          :value="item.value"
          :label="item.label"
          :key="index"
        >
        </el-option>
      </el-select>
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
    </el-form-item>
  </el-form>
</template>

<script>
<<<<<<< HEAD
import {searchDoctor, getDocument} from "../api/index"

export default {
    props: ["form"],
    data() {
        return {
            doctorName: ""
        }
    },
    methods: {
        async handleDoctorQuery(keyword, callback) {
            var data = await searchDoctor(keyword, this.form.clinic, 0);
            if (data) {
                var rs = [];
                var data = data.data;
                for(var i = 0; i < data.length; i++) {
                    var res = await getDocument("doctor", data[i]._id);
                    var value = res.position + " " + res.name;
                    rs.push({
                        value: value,
                        id: data[i]._id,
                    })
                }
                
                callback(rs);
            }
        },
        handleDoctorSelect(item) {
            this.form.doctor = item.id;
        },
    }
=======
import { searchShift } from "../api/api";
export default {
  props: ["form"],
  data() {
    return {};
  },
  methods: {
    async getShift() {
      this.form.shiftData = [];
      let data = searchShift(this.form.doctor);
      this.form.shiftData = data.data;
    },
  },
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
};
</script>
