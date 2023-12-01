def numeric(char: str):
  return char.isnumeric()


def convert_digits(string: str):
  str_nums = {
      "one": 1,
      "two": 2,
      "three": 3,
      "four": 4,
      "five": 5,
      "six": 6,
      "seven": 7,
      "eight": 8,
      "nine": 9
  }

  for num_str in list(str_nums.keys()):
    # replaces number string with number
    string = string.replace(num_str, num_str + num_str[-1])

  # for every numder as string:
  for num_str in list(str_nums.keys()):
    # replaces number string with number
    string = string.replace(num_str, str(str_nums[num_str]))

  return string


with open("input.txt", "r", encoding="utf-8") as f:
  lines = f.read().split("\n")

  for i in range(len(lines)):
    lines[i] = convert_digits(lines[i])
    # filters out all characters thare arent numeric, after conversion
    lines[i] = list(filter(numeric, lines[i]))

  # concats first and last digit in string
  for i in range(len(lines)):
    lines[i] = str(lines[i][0]) + str(lines[i][-1])

  # makes all elements in lines to integers, to sum them later
  lines = list(map(int, lines))

  # outputs processed lines, for debugging
  with open("output.txt", "w", encoding="utf-8") as g:
    g.write("\n".join(str(x) for x in lines))

  print(sum(lines))
