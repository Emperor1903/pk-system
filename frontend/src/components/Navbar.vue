<template>
<div id="navbar">
  <b-navbar toggleable="lg" type="dark" variant="info">
    <b-navbar-brand href="#">NavBar</b-navbar-brand>

    <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

    <b-collapse id="nav-collapse" is-nav>
      <b-navbar-nav>
        <b-nav-item ><router-link to="/">Trang chủ</router-link></b-nav-item>
        <b-nav-item ><router-link to="/about">Giới thiệu</router-link></b-nav-item>
        <b-nav-item >Tìm kiếm bác sĩ</b-nav-item>
        <b-nav-item >Hướng dẫn khách hàng</b-nav-item>
      </b-navbar-nav>

      <!-- Right aligned nav items -->
      <b-navbar-nav class="ml-auto">
        <b-nav-form>
          <b-form-input size="sm" class="mr-sm-2" placeholder="Search"></b-form-input>
          <!-- <b-button size="sm" class="my-2 my-sm-0" type="submit">Search</b-button> -->
          <b-button size="sm" class="my-2 my-sm-0" variant="primary" v-b-modal.modal><b>Đăng ký khám</b></b-button>
        </b-nav-form>
      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
  <b-modal id="modal" title="Đăng ký khám chữa bệnh" centered size="lg">
    <b-form >
      <b-form-group inline label="Họ và tên" label-for="fullname">
        <b-form-input id="fullname" v-model="form.fullname" required>

        </b-form-input>
      </b-form-group>
      <b-form-group label="Ngày sinh" label-for="dateofbirth">
        <b-form-datepicker :max="now" placeholder="MM/DD/YYYY" :date-format-options="{year: 'numeric', month: '2-digit', day: '2-digit'}"  id="dateofbirth" v-model="form.dateofbirth" required>

        </b-form-datepicker>
      </b-form-group>
      <b-form-group label="Điện thoại" label-for="phone">
        <b-form-input id="phone" v-model="form.phone" required>

        </b-form-input>
      </b-form-group>
      <b-form-group label="Email" label-for="email">
        <b-form-input id="email" v-model="form.email" required>

        </b-form-input>
      </b-form-group>
      <b-form-group label="Địa đểm gần bạn" label-for="location">
        <b-form-select id="location" v-model="form.location" :options="optionLocation" @change="getHospitals" required>

        </b-form-select>
      </b-form-group>
      <b-form-group label="Lựa chọn bệnh viện" label-for="hospital"  v-if="form.location != null">
        <b-form-select id="hospital" v-model="form.hospital" :options="optionHospital" @change="getDoctors" required>

        </b-form-select>
      </b-form-group>
      <b-form-group label="Lựa chọn bác sĩ" label-for="doctor" v-if="form.location != null && form.hospital != null">
        <b-form-select id="doctor" v-model="form.doctor" :options="optionDoctor" required>

        </b-form-select>
      </b-form-group>
      <b-form-group label="Ngày khám bệnh" label-for="appointmentdate">
        <b-form-datepicker :min="now" placeholder="MM/DD/YYYY" :date-format-options="{ year: 'numeric', month: '2-digit', day: '2-digit'}" id="appointmentdate" v-model="form.appointmentdate" required>

        </b-form-datepicker>
      </b-form-group>
      <b-form-group label="Nhu cầu khám bệnh" label-for="describe">
        <b-form-textarea id="describe" v-model="form.describe" required>

        </b-form-textarea>
      </b-form-group>
    </b-form>
    <b-card class="mt-3" header="Form Data Result">
      <pre class="m-0">{{ form }}</pre>
    </b-card>
  </b-modal>
</div>
</template>

<script>
export default {
    name : "Navbar",
    components: { 
  },
  data () {
    return {
      now : new Date(),
      optionLocation : [
          { value: null, text: 'Vui lòng lựa chọn địa điểm khám bệnh' },
          { value: 1, text: 'Hà Nội' },
          { value: 2, text: 'TP Hồ Chí Minh' }
      ],
      optionHospital : [],
      optionDoctor: [],
      form : {
        fullname : null ,
        dateofbirth: null,
        phone : null,
        email : null,
        location: null,
        hospital: null,
        doctor : null,
        appointmentdate: null,
        describe: null
      }
    }
  },
  methods : {
    getHospitals : function() {
      this.optionHospital = [
        {
          value: null , text : 'Vui lòng lựa chọn bệnh viện'
        },
        {
          value: 1, text: 'Bệnh viện Bạch Mai'
        },
        {
          value: 2, text: 'Bệnh viện Việt Đức'
        }
        ]
    },
    getDoctors : function() {
      this.optionDoctor = [
        {
          value: null , text : 'Vui lòng lựa chọn bác sĩ ( không bắt buộc )'
        },
        {
          value: 1, text: 'Bác sĩ A'
        },
        {
          value: 2, text: 'Bác sĩ B'
        }
      ]
    }
  }
}
</script>