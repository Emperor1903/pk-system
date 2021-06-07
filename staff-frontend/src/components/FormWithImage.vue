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
  
  <el-row type="flex" class="row-bg" justify="end">
    <el-col :span="6" :offset="6">
      <el-button @click="handleCancel">Thoát</el-button>
      <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
    </el-col>
  </el-row>
  
</div>
</template>
<script>
import {getDocument, uploadImage} from "../api/admin.js";

export default {
    name: "FormWithImage",
    props: ["state", "fields"],
    data() {
        return {
            form: {
                images: [],
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
        haveImage() {
            var fields = this.fields.filter(i => i.type === "image");
            return fields.length > 0;
        },
        searchFields() {
            var fields = this.fields.filter(i => i.type === "search");
            return fields;
        },
        visible() {
            return this.state.visible;
        },
        id() {
            return this.state.id["$oid"];
        }
    },
    watch: {
        async id(val) {
            console.log(val);
            await this.getData({"$oid": val});
        }
    },
    mounted: async function() {
        await this.getData(this.state.id);
    },
    methods: {
        async getData(id) {
            this.form._id = {
                "$oid": id
            };
            var data = await getDocument(this.state.doc, id);
            for(var t = 0; t < this.fields.length; ++t) {
                var i = this.fields[t];
                var v = data ? data[i.field] : i.value;
                if (v) {
                    this.form[i.field] = v;
                    if (i.type === "search") {
                        var response = await getDocument(i.field, v);
                        if (response)
                            this.form[i.field + "Name"] = response.name;
                    }
                    if (i.type === "image") {
                        this.form[i.field] = v.map(u => this.fImage(u));
                    }
                }
            }
            this.form = { ...this.form };
        },
        fImage(i) {
            return {
                name: i.split("/")[i.split("/").length - 1],
                url: i
            }
        },
        async handleConfirm() {
            this.$emit('confirm', this.form);
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
                    this.form.images.push(this.fImage(i));
                } else {
                }
            } finally {
                this.loading = false;
            }
        },
        handleSelect(item) {
            this.form[item.field] = item.id;
            this.form[item.field + "Name"] = item.value;
        }
    }
}
</script>
