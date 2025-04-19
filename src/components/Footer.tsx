export default function Footer() {
  return (
    <div
      id="footer"
      className="w-full bg-stone-100 dark:bg-stone-800 p-4 text-center border-t border-gray-200 dark:border-stone-700"
    >
      <div className="text-sm text-gray-600 dark:text-gray-400">
        Copyright © Harsh Narayan Jha. See Energi on{" "}
        <a
          href="https://github.com/HarshNarayanJha/energi"
          className="text-blue-500 hover:text-blue-700"
        >
          GitHub
        </a>
        . Still WIP.
      </div>
    </div>
  )
}
