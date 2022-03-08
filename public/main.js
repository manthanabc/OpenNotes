var vm = Vue.createApp({
  //el: '#cards',
  data() {
    return {
    a: 1,
    b: [1,2,3,4],
    selected_category: '',
    state: {
	    files: [
{name: "Electromagnatism", type: "text" , link: "", category: "physcis"},
{name: "Gravity", type: "text" , link: "", category: "physcis"},
{name: "Calculus", type: "text" , link: "", category: "math"},
{name: "Algebra", type: "text" , link: "", category: "math"},
{name: "Vector Calculus", type: "text" , link: "", category: "math"},
{name: "Volcanoes", type: "text" , link: "", category: "geography"},
{name: "DataStructures", type: "text" , link: "", category: "computers"},
{name: "Blockchains", type: "text" , link: "", category: "computers"},
{name: "Os dev", type: "text" , link: "", category: "computers"},
{name: "human heart", type: "text" , link: "", category: "biology"},
	    ]
    }
  }
  },
  computed: {
    // a computed getter
    b: function () {
      // `this` points to the vm instance
      return this.a + 1
    },
  },
  methods: {
	open_window: function(e) {
		window.open("ipfs://ipfs/"+e);
        },
	make_primary: function(e) {
		if (e.srcElement.id == 'New') {
		 document.getElementById('Pins').className = '';
		 document.getElementById('Explore').className='';
		}else if (e.srcElement.id == "Pins") {
		 document.getElementById('New').className = '';
		 document.getElementById('Explore').className = '';
		} else {
		 document.getElementById('Pins').className = '';
		 document.getElementById('New').className='';
		}
		e.srcElement.className="has-background-primary";
		selected_category = e.srcElement.id;
	}
  },
  async mounted() {
     console.log("called")
     // this.counter=6
     a=await fetch("/data")
     b=await a.text()
     bn = b.replaceAll("\u0000","").substring(44);
	       console.log(bn);
     this.state.files=JSON.parse(bn);
     //this.series=b.series
    }
}
).mount("#app")
