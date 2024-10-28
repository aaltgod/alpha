import { defineComponent } from 'vue'
import { useRouter } from '#app'


export default defineComponent({
	name: 'IndexPage',
	setup() {
		const $router = useRouter()

		$router.replace(`/streams`)

		return () => (
			<></>
		)
	},
})
