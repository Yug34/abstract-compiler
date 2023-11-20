using namespace std;

class ExampleClass: public RandomClass {
    public:
        string randomProperty;
        //std::cout also parses correctly
        void printname() { std::cout << "Some string" << randomProperty; }
};
int main() {
    ExampleClass obj1;
    obj1.randomProperty = "Example String";
    obj1.printname();
    return 0;
}

bool f = !!true;

int a = b;
if (a == b) {}
if (a != b) {}
int c = a / b;