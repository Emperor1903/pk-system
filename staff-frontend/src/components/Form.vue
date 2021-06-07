<template>
<div v-loading="loading">
  <el-form :model="form">
    <el-form-item
      :label="value.name"
      :label-width="formLabelWidth"
      v-for="(value, index) in inputFields"
      :key="value.field">
      <el-input
        v-model="form[value.field]"
        autocomplete="off">
      </el-input>
    </el-form-item>
    <el-form-item  
      :label="value.name"
      :label-width="formLabelWidth"
      v-for="(value, index) in searchFields"
      :key="value.field">
      <el-autocomplete
        v-model="form[value.field + 'Name']"
        :fetch-suggestions="value.query"
        @select="handleSelect"
        placeholder="Tìm kiếm">
      </el-autocomplete>
    </el-form-item>
    <el-form-item :label="value.name"
                  :label-width="formLabelWidth"
                  v-for="(value, index) in areaFields"
                  :key="value.field">
      <el-input
        type="textarea"
        :autosize="{ minRows: value.row, maxRows: value.row}"
        :placeholder="value.hint"
        v-model="form[value.field]">
      </el-input>
    </el-form-item>
    <el-form-item :label="value.name"
                  :label-width="formLabelWidth"
                  v-for="(value, index) in optionFields"
                  :key="value.field">
      <el-select v-model="form[value.field]" placeholder="Select">
        <el-option
          v-for="item in value.options"
          :key="item.value"
          :label="item.label"
          :value="item.value">
        </el-option>
      </el-select>
    </el-form-item>
  </el-form>
  <el-row type="flex" class="row-bg" justify="end">
      <el-button @click="handleCancel">Thoát</el-button>
      <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
  </el-row>
  
</div>
</template>
<script>
import {getDocument} from "../api/admin.js";

export default {
    name: "Form",
    props: ["state", "fields"],
    data() {
        return {
            form: {
            },
            loading: false,
            formLabelWidth: "120px",
        };
    },
    computed: {
        inputFields() {
            var fields = this.fields.filter(i => i.type === "input");
            return fields;
        },
        areaFields() {
            var fields = this.fields.filter(i => i.type === "area");
            return fields;
        },
        searchFields() {
            var fields = this.fields.filter(i => i.type === "search");
            return fields;
        },
        optionFields() {
            var fields = this.fields.filter(i => i.type === "option");
            return fields;
        },
        visible() {
            return this.state.visible;
        }
    },
    mounted: async function() {
        var data = await getDocument(this.state.doc, this.state.id);
        for(var t = 0; t < this.fields.length; ++t) {
            var i = this.fields[t];
            var v = data ? data[i.field] : i.value;
            if (v) {
                this.form[i.field] = v;
                if (i.type === "search") {
                    var response = await getDocument(i.field, v);
                    if (response) 
                        this.form[i.field + "Name"] = response.name;
                    console.log(this.form);
                }
            }
        }
        this.form = { ...this.form };
    },
    methods: {

        async handleConfirm() {
            this.$emit('confirm', this.form);
        },
        async handleCancel() {
            this.state.visible = false;
        },
        handleSelect(item) {
            this.form[item.field] = item.id;
            this.form[item.field + "Name"] = item.value;
        }
    }
}
</script>
