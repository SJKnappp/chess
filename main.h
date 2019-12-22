#include <iostream>
#include <ostream>
#include <string>
#include <vector>

struct peice{
  int player = 0; //not played = 0 black =1 white =2
  bool moved = 0;
  char state = ' ';
};

struct board{
  peice tiles[8][8];
  std::vector<std::string> PGN;
};


namespace Colour{
  enum Code{
    FG_BLUE =34,
    FG_DEFAULT = 39,
    BG_DEFAULT = 49
  };
  class Modifier{
    Code code;
  public:
    Modifier(Code pCode) : code(pCode) {}
    friend std::ostream&
    operator<<(std::ostream& os, const Modifier& mod)  {
      return os << "\033[" << mod.code << "m";
    }
  };
}


int main();
void print(board board);
std::string allowedMove(board board, std::string move, bool isWhite);
board intialise();
