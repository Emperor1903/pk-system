<template>
<<<<<<< HEAD
<el-form label-width="120px">
  <el-form-item label="Lựa chọn ca khám">
    <el-autocomplete
      :fetch-suggestions="handleShiftQuery"
      @select="handleShiftSelect"
      v-model="shiftName"
      class="select" />
  </el-form-item>
</el-form>
=======
  <el-form>
    <el-form-item label="Ca khám">
      <el-select v-model="form.shift">
        <el-option
          v-for="(item, index) in form.shiftData"
          :value="item.value"
          :label="item.label"
          :key="index"
        >
      </el-select>
    </el-form-item>
    
  </el-form>
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
</template>

<script>
import {searchShift, getDocument} from "../api/index"

export default {
    props: ["form"],
    data() {
        return {
            shiftName: ""
        }
    },
    methods: {
        async handleShiftQuery(keyword, callback) {
            var data = await searchShift(keyword, this.form.doctor, null, 0);
            if (data) {
                var rs = data.data.map(i => {
                    return {
                        value: i.name,
                        id: i._id,
                    }});
                callback(rs);
            }
        },
        handleShiftSelect(item) {
            this.form.shift = item.id;
        },
    }
}
</script>
