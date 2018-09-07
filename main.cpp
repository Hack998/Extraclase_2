#include <iostream>

using namespace std;

class Engine
{

public:
    void Start() { }

};

class Headlights
{

public:
    void TurnOn() { }

};

//  Facade
class Car
{

private:
    Engine engine;
    Headlights headlights;

public:
    void TurnIgnitionKeyOn()
    {
        headlights.TurnOn();
        engine.Start();
        cout << "The car is on" << endl;
    }

};

int main() {
    // Se consume el facade
    Car car{};
    car.TurnIgnitionKeyOn();

    return 0;
}