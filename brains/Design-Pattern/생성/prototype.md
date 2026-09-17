# 프로토타입 패턴

프로토타입은 실제 제품을 만들기에 앞서 테스트를 위한 샘플 제품을 만드는데 이때, 샘플 제품을 프로토타입이라고 칭한다.
  
프로토타입 패턴은 객체를 생성하는데 비용이 많이 들고, 비슷한 객체가 이미 있는 경우에 사용되는 생선 패턴 중 하나다.
  
즉, 프로토타입 패턴은 원본 객체를 새로운 객체에 복사하여 필요에 따라 수정하는 메커니즘을 제공한다.
  
프로토타입 패턴은 복사를 위해 자바에서 제공하는 clone 메서드를 사용한다.

![](https://velog.velcdn.com/images%2Fnewtownboy%2Fpost%2Fc06b7237-e6d4-4061-af91-fc875c425745%2Fimage.png)

## 프로토타입 패턴은 생성패턴이다.

생성 패턴은 인스턴스를 만드는 절차를 추상화하는 패턴이다.  
  
생성 패턴에 속하는 패턴들은 객체를 생성, 합성하는 방법이나 객체의 표현방법을 시스템과 분리해준다.
  
생성패터은 시스템이 상속보다 복합방법을 사용하는 방향으로 진화하면서 더욱 중요해지고 있다.
  
생성패턴에는 두 가지의 이슈가 존재한다.
1. 생성 패턴은 시스템이 어떤 Concrete Class를 사용하는지에 대한 정보를 캡슐화한다.
2. 생성 패턴은 이들 클래스의 인스턴스들이 어떻게 만들고 어떻게 결합하는지에 대한 부분을 완전히 가려준다.

즉, 생성패턴을 사용하면 무엇이 생성되고, 누가 이것을 생성햇으며, 이것이 어떻게 생성되는지, 언제 생성할 것인지 결정하는데 유연성을 확보할 수 있다.

## 프로토타입 패턴 구현
프로토타입 패턴은 객체를 생성하는데 비용이 들고, 이미 유사한 객체가 존재하는 경우에 사용된다.
  
자바에서 제공하는 clone 메서드를 사용하기 때문에 생성하고자 하는 객체 clone에 대한 Override를 요구한다. 이때, 주의할 점은 반드시 생성하고자 하는 객체의 클래스에서 clone 메서드가 정의되어있어야한다.
  
예를 들어, DB로 부터 가져온 데이터를 프로그램에서 수차례 수정을 해야하는 요구사항이 있는 경우 매번 new 키워드를 사용해 객체를 생성하여 DB로부터 모든 데이터를 가져오는 것은 좋은 아이디어가 아니다.
  
그 이유는 DB로 접근해서 데이터를 가져오는 행위는 비용이 크기 때문이다.  
따라서 한 번 DB에 접근하여 데이터를 가져온 객체를 필요에 따라 새로운 객체에 복사하여 데이터 수정 작업을 하는 것이 더 좋은 방법이다.

```java
// Employees.java
public class Employees implements Cloneable {
    private List <String> empList;

    public Employees() {
        empList = new ArrayLis<>();
    }

    public Employees(List < String > list) {
        this.empList = list;
    }

    public void loadData() {
        empList.add("Ann");
        empList.add("David");
        empList.add("John");
        empList.add("Methew");
    }

    @Override
    public Object clone() throws CloneNotSupportedException {
        List<String> temp = new ArrayList<>();
        for (String str: this.empList) {
            temp.add(str);
        }
        return new Employees(temp);
    }
}
```

위 코드는 생성자의 종류를 두 가지로 구분한다.
  
첫 번째는 매개변수가 없는 기본 생성자이고, 새로운 객체를 생성한다.
  
두 번째는 리스트의 매개변수가 있는 생성자이고, 현재 정의된 empList 객체에 전달받은 list를 저장한다.
  
loadData 메서드를 통해 DB에서 데이터를 불러온 것과 같은 효과를 주었고, getEmpList 메서드를 통해 empList를 반환하였다.
  
clone 메서드는 앞서 말했듯이 오버라이드를 통해 구현하며, 새로운 객체 temp를 생성하여 기존에 empList에 존재하던 데이터를 temp 리스트에 추가해주고 그것을 두 번째 생성자에게 반환한다.

```java
// PrototypePattern.java
public class PrototypePattern {
    public static void main(String[] args) throws CloneNotSupportedException {
        Employees emps = new Employees();
        emps.loadData(); // Ann, John, Methew...

        Employees emps1 = (Employees) emps.clone();
        Employees emps2 = (Employees) emps.clone();

        List<String> list1 = emps1.getEmpList();
        list1.add("Peter");

        List<String> list2 = emps2.getEmpList();
        list2.remove("John");

        System.out.println("emps: " + emps.getEmpList());
        System.out.println("emps1: " + list1.getEmpList());
        System.out.println("emps2: " + list2.getEmpList());
    }
}
```

```
// 결과화면
emps List  : [Ann, David, John, Methew]
emps1 List : [Ann, David, John, Methew, Peter]
emps2 List : [Ann, David, Methew]
```

Employees 클래스에서 clone 메서드를 제공하지 않았다면, DB로부터 매번 employee 리스트를 가져와야하고 이는 곧 큰 비용 발생으로 연결될 것이다.
  
하지만 프로토타입패턴을 사용한다면 1회의 DB접근을 통해 가져온 데이터를 다른 객체에 복사하여 사용하면 비용적인 부분을 절감할 수 있을 것이다.