import Soma from '../icons/Soma.tsx'

export default ({ title, children }: Lume.Data, helpers: Lume.Helpers) => (
  <>
<header className="flex sticky top-0 z-50 shadow-xl whitespace-nowrap
    bg-gray-200 dark:bg-black dark:text-white
    sm:flex-col sm:items-center sm:justify-items-center
    md:flex-row">
    <nav className="sm:hidden px-3 py-2">
      <svg className="h-full w-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" strokeWidth="2" d="M1 1H13 M1 5H10 M1 9H13"/>
      </svg>
    </nav>
    <div className="flex flex-row grow justify-center items-center p-2 sm:pb-0 md:pb-2">
      <a href="/" aria-current="page">
        <Soma className="w-10 h-10 rounded-full bg-black dark:bg-white text-white dark:text-black" />
      </a>
      <div className="text-lg font-banner whitespace-nowrap overflow-x-auto px-3">SOMA EASTSIDE CHURCH</div>
    </div>
    <nav className="hidden sm:block flex-none">
      <ul className="flex flex-row h-full">
        <li>
          <details className="relative h-full">
            <summary className="h-full px-4 flex items-center">ABOUT</summary>
            <ul className="absolute shadow-xl bg-gray-200 dark:bg-black px-4 pb-2 rounded-b-lg divide-y divide-black">
              <li><a href="/content/about/mission">OUR MISSION</a></li>
              <li><details>
                <summary>OUR TEAM</summary>
                <ul className="px-3">
                  <li><a href="/content/about/team-elders">Elders</a></li>
                </ul>
              </details></li>
              <li><a href="/content/about/next-steps">NEXT STEPS</a></li>
            </ul>
          </details>
        </li>
        <li>
          <details className="relative h-full">
            <summary className="h-full px-4 flex items-center">MINISTRIES</summary>
            <ul className="absolute shadow-xl bg-gray-200 dark:bg-black px-4 pb-2 rounded-b-lg divide-y divide-black">
              <li><a href="/content/ministries/mission-communities">Missional Communities</a></li>
            </ul>
          </details>
        </li>
        <li className="px-4 flex items-center">EVENTS</li>
        <li className="px-4 flex items-center">GIVE</li>
        <li className="px-4 flex items-center">SERMONS</li>
        <li className="px-4 flex items-center m-2">
          <a className="rounded-full px-4 py-2 bg-blue-500 text-white dark:text-black" href="/content/about/connect-with-us">CONNECT</a>
        </li>
      </ul>
    </nav>
  </header>
  </>
)
