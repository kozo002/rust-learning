const numbers = [4, 1, 48, 5, 30, 2, 43, 9, 6, 8, 3, 7, 0, 12, 10, 22]

function sort(arr, up) {
  if (arr.length <= 1) {
    return arr
  }
  const mid = Math.floor(arr.length / 2)
  const first = sort(arr.slice(0, mid), true)
  const second = sort(arr.slice(mid), false)
  const arr2 = [...first, ...second]
  return subSort(arr2, up)
}

function subSort(arr, up) {
  if (arr.length <= 1) {
    return arr
  }
  compareAndSwap(arr, up)
  const mid = Math.floor(arr.length / 2)
  const first = subSort(arr.slice(0, mid), up)
  const second = subSort(arr.slice(mid), up)
  return [...first, ...second]
}

function compareAndSwap(arr, up) {
  const mid = Math.floor(arr.length / 2)
  let i = 0
  while (i < mid) {
    const left = arr[i]
    const right = arr[mid + i]
    if ((left > right) === up) {
      arr[i] = right
      arr[mid + i] = left
    }
    i += 1
  }
}

console.log(sort(numbers, true))
