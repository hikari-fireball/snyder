#include <array>
#include <iostream>
#include <iterator>
#include <list>
#include <set>
#include <sstream>
#include <vector>

// TODO add some randomness

class SudokuCell {
	public:

	bool free;
	short value;
	std::set<short> possibilities;

	friend std::ostream& operator<<(std::ostream &strm, const SudokuCell &cell);

	SudokuCell():
	free(true), value(0)
	{
		for (int number = 1; number <= 9; number++)
			possibilities.insert(number);
	}

	SudokuCell(const SudokuCell& other):
	free(other.free), value(other.value), possibilities(other.possibilities)
	{}

	bool isFree() const {
		return free;
	}

	bool isValid() const {
		return !free || !possibilities.empty();
	}

	const std::set<short> getPossibilities() const {
		return possibilities;
	}

	void removeFromPossibilities(short number) {
		if (possibilities.count(number))
			possibilities.erase(number);
	}

	void setValue(short number) {
		value = number;
		free = false;
		possibilities.clear();
	}
};


std::ostream& operator<<(std::ostream &strm, const SudokuCell &cell) {
	if (cell.isFree())
		return strm << "(" << cell.possibilities.size() << ")";
	else
		return strm << " " << cell.value << " ";
}


class SudokuGrid {
	public:

	std::array<std::array<SudokuCell, 9>, 9> cells;
	std::set<std::pair<short, short>> free_cells;

	friend std::ostream& operator<<(std::ostream &strm, const SudokuGrid &grid);

	SudokuGrid() {
		//free_cells
		for (auto line = 0; line < (int)cells.size(); line++)
			for (auto col = 0; col < (int)cells[line].size(); col++)
				free_cells.insert(std::pair<short, short>(line, col));
	}

	SudokuGrid(const SudokuGrid& other)
	:cells(other.cells), free_cells(other.free_cells)
	{}

	bool isSolved() {
		for (auto line: cells)
			for (auto cell: line)
				if (cell.isFree())
					return false;
		return true;
	}

	bool isValid() {
		for (auto line: cells)
			for (auto cell: line)
				if (!cell.isValid())
					return false;
		return true;
	}

	void applyNumber(short line, short col, short number) {
		//line
		for (auto tc = 0; tc < (int)cells[line].size(); tc++)
			cells[line][tc].removeFromPossibilities(number);
		//column
		for (auto tl = 0; tl < (int)cells.size(); tl++)
			cells[tl][col].removeFromPossibilities(number);
		//square
		for (auto tl = (line / 3) * 3; tl < (line / 3) * 3 + 3; tl++)
			for (auto tc = (col / 3) * 3; tc < (col / 3) * 3 + 3; tc++)
				cells[tl][tc].removeFromPossibilities(number);

		cells[line][col].setValue(number);
		free_cells.erase(std::pair<short, short>(line, col));
	}

	std::vector<SudokuGrid> generateOffspring() {
		std::vector<SudokuGrid> offspring;
		auto target_cell = *free_cells.begin();
		auto &cell = cells[target_cell.first][target_cell.second];
		auto possibilities = cell.getPossibilities();

		for (auto number: possibilities) {
			SudokuGrid child = SudokuGrid(*this);
			child.applyNumber(target_cell.first, target_cell.second, number);
			if (child.isValid())
				offspring.push_back(child);
		}

		return offspring;
	}
};


std::ostream& operator<<(std::ostream &strm, const SudokuGrid &grid) {
	auto &cells = grid.cells;
	std::stringstream buffer;
	for (auto line_i = cells.begin(); line_i != cells.end(); line_i++) {
		if (line_i != cells.begin())
			buffer << std::endl;
		for (auto cell_i = line_i->begin(); cell_i != line_i->end(); cell_i++) {
			if (cell_i != line_i->begin())
				buffer << " ";
			buffer << *cell_i;
		}
	}
	return strm << buffer.str();
}


void createSudoku() {
	std::list<SudokuGrid> stack;

	SudokuGrid root;
	stack.push_back(root);

	while (!stack.empty()) {
		auto current = stack.back();
		stack.pop_back();

		auto offspring = current.generateOffspring();
		for (auto child: offspring) {
			if (child.isSolved()) {
				std::cout << "solution found:" << std::endl;
				std::cout << child << std::endl;
			} else {
				stack.push_back(child);
			}
		}
	}
}


void solveSudoku() {
	std::cout << "solving" << std::endl;
}


void printHelp(std::string program_name) {
	std::cerr << "Usage: " << program_name << " <command>" << std::endl;
	std::cerr << std::endl;
	std::cerr << "<command>:" << std::endl;
	std::cerr << "\tcreate" << std::endl;
	std::cerr << "\t\tCreates a new Sudoku puzzle." << std::endl;
	std::cerr << "\tsolve" << std::endl;
	std::cerr << "\t\tSolves the sudoku puzzle specified in stdin."
		<< std::endl;
}


int main(int argc, char *argv[]) {
	if (argc != 2) {
		printHelp(argv[0]);
		exit(1);
	}
	std::string command = argv[1];

	if (command == "create") {
		createSudoku();
	} else if (command == "solve") {
		solveSudoku();
	} else {
		std::cerr << argv[0] << ": " << command << " is not a command."
			<< std::endl;
		exit(2);
	}

	return 0;
}
