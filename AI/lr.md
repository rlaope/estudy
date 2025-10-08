# 로지스틱 회귀분석 이론

회귀분석 대표 모형중 하나인 로지스틱 회귀분석에 대해서 설명해보겠다.

이제까지 다뤘던 선형 회귀모델과는 다르게 로지스틱 회귀분석의 경우에는 **분류기법**으로 사용된다.

### GLM 

일반화 선형모형 Generailzed Linear Model을 먼저 보면 GLM의 가정을 독립성, 정규성, 등분산성, 선형성을 말하고 있는데,

이러한 기본 가정을 지키지 못할때 사용할 수 있는것이 GLM이다.

대표적으로 종속변수가 정규분포를 따르지 못할때, 연속형이 아닌 범주형일때를 생각할 수 있겠다.

이런 기본 가정을 만족하지 못할때 우리는 link function이라는 개념으로 일반 선형 모형으로 확장이 가능하고

모형을 선정하는 것에 있어서는 종속변수의 분포와 link function을 지정함으로써 진행할 수 있는데, 

로지스틱 회귀 분석은 glm이 하나로 종속 변수가 이항분포를 따른다고 가정하고 logit함수를 활용한 케이스다.

## 로지스틱 회귀분석

종속변수가 두 개의 클래스를 갖는 범주형일 때 분류 기법으로 사용된다.

근데 분류를 하는데 있어서는 확률값을 통해서 이루어진다. 예를들어 50퍼센트 이상의 값은 1 아니면 0으로 분류하는 방식으로.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTExMTJfNDMg/MDAxNTczNTU1MDkwNDY5.MyBJMttJrsw7U9ESxKDUVZ1cC9qoCfgh2z4-xvoN6tQg.B3bQ8yOJftHjRglscntPp2NgayhZ2Qmad8l2r1qNPlYg.PNG.winddori2002/1.PNG?type=w800)

예를들면 1시간을 공부했을때 시험의 합격할 확률 10퍼센트 2시간은 24퍼센트 3시간은 35퍼센트 9시간은 83퍼센트 이런 데이터들이 있을때

시간당 합격여부를 1, 0으로 분류하고싶을때 로지스틱 회귀분석 모델을 사용할 수 있다.

이것을 구하기 위한 회귀식은 아래와 같이 표현이 가능하다

$Y\ \left(확률\right)=\ \beta _0+\beta _1X_1+\beta _2X_2+...+\beta _nX_n$Y 

근데 여기서 문제가 발생하는데, 종속변수의 범위는 0, 1이지만 독립변수는  [-∞,∞] 라는것

음과 같은 일반 선형식이 표현됐을때는 항상 확률값이 0, 1 사이에 있을 것이라는 것을 보장할 수 가 없다.

이를 해결하기 위해 링크함수를 사용하는데 로지스틱회귀에선느 구체적으로 시그모이드 함수를 씌워 확률값을 반환한다.

간단하게 말해서 음수가 되면 0, 1을 초과하면 1로 표현하는거로 생각하면 쉽다.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTExMTNfODgg/MDAxNTczNjE4NDA1MDQz.7efEKnxyEaW52i5AqBM_hPFxf3aaANuU9NgMdzEXFGcg.V5vFVOSfFydEesSqdzk84kpskDgqbcBMuhrYbCloFEAg.PNG.winddori2002/1.PNG?type=w800)

이런 형태를 띄우고 있음

### AUROC (Area Under ROC)

정확도는 threshold에 따라 변하기 때문에 지표로서 부족할 대가 있다.

예를 들어 49퍼센트 확률로 합격이라 불합격으로 판단했는데 진짜 데이터는 합격했을수도 있다.

이를 보완하기 위해 threshold에 의해 값이 변하지 않는 지표가 auroc

분류 모델의 성능을 평가하는 지표중 하나로 roc curve의 아래 면적을 계산한 값

roc curve는 분류 모델의 threshold를 변화시키면서 분류 모델의 성능을 시각적으로 나타낸 그래프로 곡선은 false positive rate(fpr)을 x축으로 tpr을 y축으로 나타낸다.

![](https://velog.velcdn.com/images/zlddp723/post/0bed4c6b-28bd-4181-863b-877815bfdbe8/image.png)

fp fn tp tn 간단하게 알아보는 짤

![](https://miro.medium.com/v2/resize:fit:720/format:webp/1*mL-nYY6MFhiG0uoR5kJaCA.jpeg)

- true positive: 참으로 판단했고 진짜 참으로 뽑음
- false positive: 참으로 판단했으나 값은 거짓으로 뽑음
- true negative: 거짓으로 판단했고 진짜 거짓임
- false negative: 거짓으로 판단했으나 참임.

True Positive Ratio(TPR)은 분류 모델에서 실제 Positive인 데이터 중에서 모델이 Positive로 예측한 데이터의 비율을 나타내는 지표다.

TPR은 민감도(Sensitivity) 또는 재현율(Recall)이라고도 불린다.

반대로 FPR은 분류 모델에서 실제 negative인 데이터중에서 모델이 positive로 잘못 예측한 데이터 비율을 나타내는 지표로 fpr은 1에서 특이도를 뺀 값과 같다.

fpr은 0에 가까울수록 분류 모델이 negative로 예측한 것과 실제 negative인 것이 일치하는 것으로 좋다고 평가되지만, 1에 가까울수록 성능이 나쁜것으로 판단된다.

> Threshold의 정의는 확률값을 범주형으로 변환할 때 기준이다. 로지스틱 회귀 모델에서는 종속 변수의 확률값을 0또는 1 사이의 값으로 출력하는데 이 확률값은 임계값 threshold 기준으로 변환됨, 예를들어 합격여부를 판단하는게 50퍼 이상이라면 50이 threshold

