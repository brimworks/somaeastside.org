export default ({ title, children }: Lume.Data, helpers: Lume.Helpers) => (
    <>
  <template id="web-carousel">
    <div className="container relative mx-auto aspect-video">
        <button
            onclick="event.target.closest('web-carousel').jumpRelative(-1)"
            className="absolute top-0 start-0 z-30 flex items-center justify-center h-full px-4 cursor-pointer group focus:outline-none">
            <span className="inline-flex items-center justify-center w-10 h-10 rounded-full bg-white/30 dark:bg-gray-800/30 group-hover:bg-white/50 dark:group-hover:bg-gray-800/60 group-focus:ring-4 group-focus:ring-white dark:group-focus:ring-gray-800/70 group-focus:outline-none">
            <svg className="w-4 h-4 text-white dark:text-gray-800 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
                <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 1 1 5l4 4"/>
            </svg>
            <span className="sr-only">Previous</span>
            </span>
        </button>
        <div className="relative rounded-lg h-full overflow-hidden bg-black">
            <div data-carousel-item="center" className="absolute h-full duration-700 ease-in-out w-full flex items-center"
                data-class-left="-translate-x-full z-10 invisible"
                data-class-right="translate-x-full z-10 invisible"
                data-class-center="translate-x-0 z-20">
                <slot className="block w-full m-0 p-0"></slot>
            </div>
        </div>
        <div className="absolute z-30 flex -translate-x-1/2 space-x-3 rtl:space-x-reverse bottom-5 left-1/2">
            <button data-carousel-jump="0"
                onclick="event.target.closest('web-carousel').jump(event.target.getAttribute('data-carousel-jump'))"
                className="w-3 h-3 rounded-full hover:ring-2 hover:ring-white/70 dark:hover:ring-black/70"
                data-class-center="bg-white/60 dark:bg-gray-800/60 ring-2 ring-gray-200/70 dark:ring-gray-800/70"
                data-class-left="bg-white/40 dark:bg-gray-800/40"
                data-class-right="bg-white/40 dark:bg-gray-800/40">
            </button>
        </div>
        <button
            onclick="event.target.closest('web-carousel').jumpRelative(1)"
            className="absolute top-0 end-0 z-30 flex items-center justify-center h-full px-4 cursor-pointer group focus:outline-none">
            <span className="inline-flex items-center justify-center w-10 h-10 rounded-full bg-white/30 dark:bg-gray-800/30 group-hover:bg-white/50 dark:group-hover:bg-gray-800/60 group-focus:ring-4 group-focus:ring-white dark:group-focus:ring-gray-800/70 group-focus:outline-none">
            <svg className="w-4 h-4 text-white dark:text-gray-800 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
                <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="m1 9 4-4-4-4"/>
            </svg>
            <span className="sr-only">Next</span>
            </span>
        </button>
    </div>
</template>
<script defer src="/script/component/web-carousel.js"></script>
</>
)
